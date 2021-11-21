use crate::nodes::Node;
use llvm_sys::prelude::*;
use llvm_sys::{core, target};
use std::ffi::{CStr, CString};
use std::fs;

pub struct Compiler {
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
    double_type: LLVMTypeRef,
    int32_type: LLVMTypeRef, // main func int type
    int8_type: LLVMTypeRef,  // for int8 ptr type
}

impl Compiler {
    pub fn new(filename: Option<&str>) -> Self {
        let cstr = match filename {
            Some(s) => CString::new(s).unwrap(),
            None => CString::new("calcead module").unwrap(),
        };
        unsafe {
            if target::LLVM_InitializeNativeTarget() != 0 {
                panic!("Could not initialize LLVM target");
            }
            if target::LLVM_InitializeNativeAsmPrinter() != 0 {
                panic!("Could not initialize LLVM ASM Printer");
            }
        }
        let context = unsafe { core::LLVMGetGlobalContext() };
        Self {
            context,
            module: unsafe { core::LLVMModuleCreateWithNameInContext(cstr.as_ptr(), context) },
            builder: unsafe { core::LLVMCreateBuilderInContext(context) },
            double_type: unsafe { core::LLVMDoubleTypeInContext(context) },
            int32_type: unsafe { core::LLVMInt32TypeInContext(context) },
            int8_type: unsafe { core::LLVMInt8TypeInContext(context) },
        }
    }

    pub fn compile(&mut self, tree: Box<Node>) -> LLVMModuleRef {
        let main_function_type = unsafe {
            core::LLVMFunctionType(
                core::LLVMInt32TypeInContext(self.context),
                std::ptr::null_mut(),
                0 as u32,
                0,
            )
        };

        let main_function_name = CStr::from_bytes_with_nul(b"main\0").unwrap();
        let main_function = unsafe {
            core::LLVMAddFunction(self.module, main_function_name.as_ptr(), main_function_type)
        };

        // Create basic block
        let block_name = CStr::from_bytes_with_nul(b"entry\0").unwrap();
        let entry_block = unsafe {
            core::LLVMAppendBasicBlockInContext(self.context, main_function, block_name.as_ptr())
        };

        // Position builder to new entry block
        unsafe {
            core::LLVMPositionBuilderAtEnd(self.builder, entry_block);
        }

        let printf_function_type = unsafe {
            let int8_ptr_type = core::LLVMPointerType(self.int8_type, 0);
            let mut param_types = [int8_ptr_type];
            core::LLVMFunctionType(self.int32_type, param_types.as_mut_ptr(), 1, 1)
        };

        let printf_function_name = CStr::from_bytes_with_nul(b"printf\0").unwrap();
        let printf_function = unsafe {
            core::LLVMAddFunction(
                self.module,
                printf_function_name.as_ptr(),
                printf_function_type,
            )
        };

        // Fill in body of function
        unsafe {
            let global_str = core::LLVMBuildGlobalStringPtr(
                self.builder,
                CStr::from_bytes_with_nul(b"%f\0").unwrap().as_ptr(),
                CStr::from_bytes_with_nul(b".str\0").unwrap().as_ptr(),
            );
            let mut printf_args = [global_str, self.visit(tree)];

            core::LLVMBuildCall(
                self.builder,
                printf_function,
                printf_args.as_mut_ptr(),
                2,
                printf_function_name.as_ptr(),
            );
            core::LLVMBuildRet(self.builder, core::LLVMConstInt(self.int32_type, 0, 0));
        }

        self.module
    }

    pub fn visit(&mut self, node: Box<Node>) -> LLVMValueRef {
        use Node::*;
        match *node {
            NumberNode(x) => unsafe { core::LLVMConstReal(self.double_type, x.into()) },
            AddNode(a, b) => unsafe {
                core::LLVMBuildFAdd(
                    self.builder,
                    self.visit(a),
                    self.visit(b),
                    CStr::from_bytes_with_nul(b"addtmp\0").unwrap().as_ptr(),
                )
            },
            SubtractNode(a, b) => unsafe {
                core::LLVMBuildFSub(
                    self.builder,
                    self.visit(a),
                    self.visit(b),
                    CStr::from_bytes_with_nul(b"subtmp\0").unwrap().as_ptr(),
                )
            },
            MultiplyNode(a, b) => unsafe {
                core::LLVMBuildFMul(
                    self.builder,
                    self.visit(a),
                    self.visit(b),
                    CStr::from_bytes_with_nul(b"multmp\0").unwrap().as_ptr(),
                )
            },
            DivideNode(a, b) => unsafe {
                core::LLVMBuildFDiv(
                    self.builder,
                    self.visit(a),
                    self.visit(b),
                    CStr::from_bytes_with_nul(b"divtmp\0").unwrap().as_ptr(),
                )
            },
            PlusNode(node) => unsafe {
                core::LLVMBuildFMul(
                    self.builder,
                    self.visit(node),
                    core::LLVMConstReal(self.double_type, 1.0),
                    CStr::from_bytes_with_nul(b"multmp\0").unwrap().as_ptr(),
                )
            },
            MinusNode(node) => unsafe {
                core::LLVMBuildFMul(
                    self.builder,
                    self.visit(node),
                    core::LLVMConstReal(self.double_type, -1.0),
                    CStr::from_bytes_with_nul(b"multmp\0").unwrap().as_ptr(),
                )
            },
        }
    }

    #[allow(dead_code)]
    pub fn dump(&mut self) {
        unsafe { core::LLVMDumpModule(self.module) }
    }

    pub fn dump_to_file(&mut self, filename: &str) -> std::io::Result<()> {
        let c_buf = unsafe { core::LLVMPrintModuleToString(self.module) };
        let c_str = unsafe { CStr::from_ptr(c_buf) };
        let str_slice = c_str.to_str().unwrap();
        fs::write(filename, str_slice)
    }
}

impl Drop for Compiler {
    fn drop(&mut self) {
        unsafe { core::LLVMDisposeBuilder(self.builder) };
        unsafe { core::LLVMDisposeModule(self.module) };
    }
}
