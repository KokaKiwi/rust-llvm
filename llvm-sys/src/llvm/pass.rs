pub enum PassKind {
    PTBasicBlock,
    PTRegion,
    PTLoop,
    PTFunction,
    PTCallGraphSCC,
    PTModule,
    PTPassManager,
}
impl PassKind {
    pub fn from_ffi(value: ::ffi::llvm_PassKind) -> PassKind {
        match value {
            ::ffi::llvm_PassKind::PT_BasicBlock => PassKind::PTBasicBlock,
            ::ffi::llvm_PassKind::PT_Region => PassKind::PTRegion,
            ::ffi::llvm_PassKind::PT_Loop => PassKind::PTLoop,
            ::ffi::llvm_PassKind::PT_Function => PassKind::PTFunction,
            ::ffi::llvm_PassKind::PT_CallGraphSCC => PassKind::PTCallGraphSCC,
            ::ffi::llvm_PassKind::PT_Module => PassKind::PTModule,
            ::ffi::llvm_PassKind::PT_PassManager => PassKind::PTPassManager,
        }
    }
    pub fn to_ffi(self) -> ::ffi::llvm_PassKind {
        match self {
            PassKind::PTBasicBlock => ::ffi::llvm_PassKind::PT_BasicBlock,
            PassKind::PTRegion => ::ffi::llvm_PassKind::PT_Region,
            PassKind::PTLoop => ::ffi::llvm_PassKind::PT_Loop,
            PassKind::PTFunction => ::ffi::llvm_PassKind::PT_Function,
            PassKind::PTCallGraphSCC => ::ffi::llvm_PassKind::PT_CallGraphSCC,
            PassKind::PTModule => ::ffi::llvm_PassKind::PT_Module,
            PassKind::PTPassManager => ::ffi::llvm_PassKind::PT_PassManager,
        }
    }
}
impl Copy for PassKind {}
pub enum PassManagerType {
    PMTUnknown,
    PMTModulePassManager,
    PMTCallGraphPassManager,
    PMTFunctionPassManager,
    PMTLoopPassManager,
    PMTRegionPassManager,
    PMTBasicBlockPassManager,
    PMTLast,
}
impl PassManagerType {
    pub fn from_ffi(value: ::ffi::llvm_PassManagerType) -> PassManagerType {
        match value {
            ::ffi::llvm_PassManagerType::PMT_Unknown => PassManagerType::PMTUnknown,
            ::ffi::llvm_PassManagerType::PMT_ModulePassManager => PassManagerType::PMTModulePassManager,
            ::ffi::llvm_PassManagerType::PMT_CallGraphPassManager => PassManagerType::PMTCallGraphPassManager,
            ::ffi::llvm_PassManagerType::PMT_FunctionPassManager => PassManagerType::PMTFunctionPassManager,
            ::ffi::llvm_PassManagerType::PMT_LoopPassManager => PassManagerType::PMTLoopPassManager,
            ::ffi::llvm_PassManagerType::PMT_RegionPassManager => PassManagerType::PMTRegionPassManager,
            ::ffi::llvm_PassManagerType::PMT_BasicBlockPassManager => PassManagerType::PMTBasicBlockPassManager,
            ::ffi::llvm_PassManagerType::PMT_Last => PassManagerType::PMTLast,
        }
    }
    pub fn to_ffi(self) -> ::ffi::llvm_PassManagerType {
        match self {
            PassManagerType::PMTUnknown => ::ffi::llvm_PassManagerType::PMT_Unknown,
            PassManagerType::PMTModulePassManager => ::ffi::llvm_PassManagerType::PMT_ModulePassManager,
            PassManagerType::PMTCallGraphPassManager => ::ffi::llvm_PassManagerType::PMT_CallGraphPassManager,
            PassManagerType::PMTFunctionPassManager => ::ffi::llvm_PassManagerType::PMT_FunctionPassManager,
            PassManagerType::PMTLoopPassManager => ::ffi::llvm_PassManagerType::PMT_LoopPassManager,
            PassManagerType::PMTRegionPassManager => ::ffi::llvm_PassManagerType::PMT_RegionPassManager,
            PassManagerType::PMTBasicBlockPassManager => ::ffi::llvm_PassManagerType::PMT_BasicBlockPassManager,
            PassManagerType::PMTLast => ::ffi::llvm_PassManagerType::PMT_Last,
        }
    }
}
impl Copy for PassManagerType {}
pub type BasicBlockPassInner = ::ffi::llvm_BasicBlockPass;

pub trait BasicBlockPassExt: ::llvm::pass::PassExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut BasicBlockPassInner;
}

pub struct BasicBlockPass {
    inner: ::core::nonzero::NonZero<*mut BasicBlockPassInner>,
    owned: bool,
}
impl ::llvm::pass::PassExt for BasicBlockPass {
    fn inner(&self) -> *mut ::ffi::llvm_Pass {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl BasicBlockPassExt for BasicBlockPass {
    fn inner(&self) -> *mut BasicBlockPassInner {
        *self.inner
    }
}
impl BasicBlockPass {
    pub unsafe fn from_inner(inner: *mut BasicBlockPassInner, owned: bool) -> BasicBlockPass {
        BasicBlockPass {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for BasicBlockPass {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Pass_delete(::llvm::pass::PassExt::inner(self));
            }
        }
    }
}
pub type CallGraphSCCPassInner = ::ffi::llvm_CallGraphSCCPass;

pub trait CallGraphSCCPassExt: ::llvm::pass::PassExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut CallGraphSCCPassInner;
}

pub struct CallGraphSCCPass {
    inner: ::core::nonzero::NonZero<*mut CallGraphSCCPassInner>,
    owned: bool,
}
impl ::llvm::pass::PassExt for CallGraphSCCPass {
    fn inner(&self) -> *mut ::ffi::llvm_Pass {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl CallGraphSCCPassExt for CallGraphSCCPass {
    fn inner(&self) -> *mut CallGraphSCCPassInner {
        *self.inner
    }
}
impl CallGraphSCCPass {
    pub unsafe fn from_inner(inner: *mut CallGraphSCCPassInner, owned: bool) -> CallGraphSCCPass {
        CallGraphSCCPass {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for CallGraphSCCPass {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Pass_delete(::llvm::pass::PassExt::inner(self));
            }
        }
    }
}
pub type FunctionPassInner = ::ffi::llvm_FunctionPass;

pub trait FunctionPassExt: ::llvm::pass::PassExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut FunctionPassInner;
}

pub struct FunctionPass {
    inner: ::core::nonzero::NonZero<*mut FunctionPassInner>,
    owned: bool,
}
impl ::llvm::pass::PassExt for FunctionPass {
    fn inner(&self) -> *mut ::ffi::llvm_Pass {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl FunctionPassExt for FunctionPass {
    fn inner(&self) -> *mut FunctionPassInner {
        *self.inner
    }
}
impl FunctionPass {
    pub unsafe fn from_inner(inner: *mut FunctionPassInner, owned: bool) -> FunctionPass {
        FunctionPass {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for FunctionPass {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Pass_delete(::llvm::pass::PassExt::inner(self));
            }
        }
    }
}
pub type LoopPassInner = ::ffi::llvm_LoopPass;

pub trait LoopPassExt: ::llvm::pass::PassExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut LoopPassInner;
}

pub struct LoopPass {
    inner: ::core::nonzero::NonZero<*mut LoopPassInner>,
    owned: bool,
}
impl ::llvm::pass::PassExt for LoopPass {
    fn inner(&self) -> *mut ::ffi::llvm_Pass {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl LoopPassExt for LoopPass {
    fn inner(&self) -> *mut LoopPassInner {
        *self.inner
    }
}
impl LoopPass {
    pub unsafe fn from_inner(inner: *mut LoopPassInner, owned: bool) -> LoopPass {
        LoopPass {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for LoopPass {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Pass_delete(::llvm::pass::PassExt::inner(self));
            }
        }
    }
}
pub type ModulePassInner = ::ffi::llvm_ModulePass;

pub trait ModulePassExt: ::llvm::pass::PassExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut ModulePassInner;
}

pub struct ModulePass {
    inner: ::core::nonzero::NonZero<*mut ModulePassInner>,
    owned: bool,
}
impl ::llvm::pass::PassExt for ModulePass {
    fn inner(&self) -> *mut ::ffi::llvm_Pass {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl ModulePassExt for ModulePass {
    fn inner(&self) -> *mut ModulePassInner {
        *self.inner
    }
}
impl ModulePass {
    pub unsafe fn from_inner(inner: *mut ModulePassInner, owned: bool) -> ModulePass {
        ModulePass {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for ModulePass {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Pass_delete(::llvm::pass::PassExt::inner(self));
            }
        }
    }
}
pub type PassInner = ::ffi::llvm_Pass;

pub trait PassExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut PassInner;

    fn do_finalization(&mut self, module: &::llvm::ModuleExt) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Pass_doFinalization(::llvm::pass::PassExt::inner(self), ::llvm::ModuleExt::inner(module));
            ret
        }
    }

    fn do_initialization(&mut self, module: &::llvm::ModuleExt) -> bool {
        unsafe {
            let ret = ::ffi::llvm::Pass_doInitialization(::llvm::pass::PassExt::inner(self), ::llvm::ModuleExt::inner(module));
            ret
        }
    }

    fn dump(&self) {
        unsafe {
            ::ffi::llvm::Pass_dump(::llvm::pass::PassExt::inner(self) as *const ::ffi::llvm_Pass);
        }
    }

    fn get_pass_kind(&self) -> ::llvm::pass::PassKind {
        unsafe {
            let ret = ::llvm::pass::PassKind::from_ffi(::ffi::llvm::Pass_getPassKind(::llvm::pass::PassExt::inner(self) as *const ::ffi::llvm_Pass));
            ret
        }
    }
}

pub struct Pass {
    inner: ::core::nonzero::NonZero<*mut PassInner>,
    owned: bool,
}
impl PassExt for Pass {
    fn inner(&self) -> *mut PassInner {
        *self.inner
    }
}
impl Pass {
    pub unsafe fn from_inner(inner: *mut PassInner, owned: bool) -> Pass {
        Pass {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for Pass {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Pass_delete(::llvm::pass::PassExt::inner(self));
            }
        }
    }
}
pub type RegionPassInner = ::ffi::llvm_RegionPass;

pub trait RegionPassExt: ::llvm::pass::PassExt {
    #[allow(non_snake_case)]
    fn inner(&self) -> *mut RegionPassInner;
}

pub struct RegionPass {
    inner: ::core::nonzero::NonZero<*mut RegionPassInner>,
    owned: bool,
}
impl ::llvm::pass::PassExt for RegionPass {
    fn inner(&self) -> *mut ::ffi::llvm_Pass {
        unsafe {
            ::core::mem::transmute(self.inner)
        }
    }
}
impl RegionPassExt for RegionPass {
    fn inner(&self) -> *mut RegionPassInner {
        *self.inner
    }
}
impl RegionPass {
    pub unsafe fn from_inner(inner: *mut RegionPassInner, owned: bool) -> RegionPass {
        RegionPass {
            inner: ::core::nonzero::NonZero::new(inner),
            owned: owned,
        }
    }
}
impl Drop for RegionPass {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ::ffi::llvm::Pass_delete(::llvm::pass::PassExt::inner(self));
            }
        }
    }
}

pub fn create_add_discriminators_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createAddDiscriminatorsPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_address_sanitizer_function_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createAddressSanitizerFunctionPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_address_sanitizer_module_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createAddressSanitizerModulePass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_aggressive_dce_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createAggressiveDCEPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_always_inliner_pass(insert_lifetime: Option<bool>) -> Option<::llvm::pass::Pass> {
    unsafe {
        let insert_lifetime = insert_lifetime.unwrap_or(false);
        let ret = ::ffi::llvm::createAlwaysInlinerPass(insert_lifetime);
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_argument_promotion_pass(max_elements: Option<u32>) -> Option<::llvm::pass::Pass> {
    unsafe {
        let max_elements = max_elements.unwrap_or(3);
        let ret = ::ffi::llvm::createArgumentPromotionPass(max_elements as ::libc::c_uint);
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_barrier_noop_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createBarrierNoopPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_block_extractor_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createBlockExtractorPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_bounds_checking_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createBoundsCheckingPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_break_critical_edges_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createBreakCriticalEdgesPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_cfg_simplification_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createCFGSimplificationPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_constant_hoisting_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createConstantHoistingPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_constant_merge_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createConstantMergePass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_constant_propagation_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createConstantPropagationPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_correlated_value_propagation_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createCorrelatedValuePropagationPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_data_flow_sanitizer_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createDataFlowSanitizerPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_dead_arg_elimination_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createDeadArgEliminationPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_dead_arg_hacking_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createDeadArgHackingPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_dead_code_elimination_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createDeadCodeEliminationPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_dead_inst_elimination_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createDeadInstEliminationPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_dead_store_elimination_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createDeadStoreEliminationPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_debug_ir_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createDebugIRPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_demote_register_to_memory_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createDemoteRegisterToMemoryPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_early_cse_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createEarlyCSEPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_flatten_cfg_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createFlattenCFGPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_function_attrs_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createFunctionAttrsPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_function_inlining_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createFunctionInliningPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_gcov_profiler_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createGCOVProfilerPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_gvn_pass(no_loads: Option<bool>) -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let no_loads = no_loads.unwrap_or(false);
        let ret = ::ffi::llvm::createGVNPass(no_loads);
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_global_dce_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createGlobalDCEPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_global_merge_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createGlobalMergePass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_global_optimizer_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createGlobalOptimizerPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_ip_constant_propagation_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createIPConstantPropagationPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_ipsccp_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createIPSCCPPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_ind_var_simplify_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createIndVarSimplifyPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_instruction_combining_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createInstructionCombiningPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_instruction_namer_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createInstructionNamerPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_instruction_simplifier_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createInstructionSimplifierPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_internalize_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createInternalizePass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_jump_threading_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createJumpThreadingPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_lcssa_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createLCSSAPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_licm_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createLICMPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_load_combine_pass() -> Option<::llvm::pass::BasicBlockPass> {
    unsafe {
        let ret = ::ffi::llvm::createLoadCombinePass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::BasicBlockPass::from_inner(ret, true))
    }
}

pub fn create_loop_deletion_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createLoopDeletionPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_loop_extractor_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createLoopExtractorPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_loop_idiom_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createLoopIdiomPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_loop_inst_simplify_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createLoopInstSimplifyPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_loop_reroll_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createLoopRerollPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_loop_rotate_pass(max_header_size: Option<i32>) -> Option<::llvm::pass::Pass> {
    unsafe {
        let max_header_size = max_header_size.unwrap_or(-1);
        let ret = ::ffi::llvm::createLoopRotatePass(max_header_size as ::libc::c_int);
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_loop_simplify_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createLoopSimplifyPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_loop_strength_reduce_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createLoopStrengthReducePass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_loop_unroll_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createLoopUnrollPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_loop_unswitch_pass(optimize_for_size: Option<bool>) -> Option<::llvm::pass::Pass> {
    unsafe {
        let optimize_for_size = optimize_for_size.unwrap_or(false);
        let ret = ::ffi::llvm::createLoopUnswitchPass(optimize_for_size);
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_lower_atomic_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createLowerAtomicPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_lower_expect_intrinsic_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createLowerExpectIntrinsicPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_lower_invoke_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createLowerInvokePass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_lower_switch_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createLowerSwitchPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_mem_cpy_opt_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createMemCpyOptPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_memory_sanitizer_pass(track_origins: Option<i32>) -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let track_origins = track_origins.unwrap_or(0);
        let ret = ::ffi::llvm::createMemorySanitizerPass(track_origins as ::libc::c_int);
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_merge_functions_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createMergeFunctionsPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_merged_load_store_motion_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createMergedLoadStoreMotionPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_meta_renamer_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createMetaRenamerPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_obj_carcap_elim_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createObjCARCAPElimPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_obj_carc_contract_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createObjCARCContractPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_obj_carc_expand_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createObjCARCExpandPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_obj_carc_opt_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createObjCARCOptPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_partial_inlining_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createPartialInliningPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_partially_inline_lib_calls_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createPartiallyInlineLibCallsPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_promote_memory_to_register_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createPromoteMemoryToRegisterPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_prune_eh_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createPruneEHPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_reassociate_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createReassociatePass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_sccp_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createSCCPPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_sroa_pass(requires_dom_tree: Option<bool>) -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let requires_dom_tree = requires_dom_tree.unwrap_or(true);
        let ret = ::ffi::llvm::createSROAPass(requires_dom_tree);
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_sample_profile_loader_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createSampleProfileLoaderPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_scalar_repl_aggregates_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createScalarReplAggregatesPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_scalarizer_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createScalarizerPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_separate_const_offset_from_gep_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createSeparateConstOffsetFromGEPPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_simple_loop_unroll_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createSimpleLoopUnrollPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_single_loop_extractor_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createSingleLoopExtractorPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_sinking_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createSinkingPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_strip_dead_debug_info_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createStripDeadDebugInfoPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_strip_dead_prototypes_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createStripDeadPrototypesPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_strip_debug_declare_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createStripDebugDeclarePass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_strip_non_debug_symbols_pass() -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let ret = ::ffi::llvm::createStripNonDebugSymbolsPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_strip_symbols_pass(only_debug_info: Option<bool>) -> Option<::llvm::pass::ModulePass> {
    unsafe {
        let only_debug_info = only_debug_info.unwrap_or(false);
        let ret = ::ffi::llvm::createStripSymbolsPass(only_debug_info);
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::ModulePass::from_inner(ret, true))
    }
}

pub fn create_structurize_cfg_pass() -> Option<::llvm::pass::Pass> {
    unsafe {
        let ret = ::ffi::llvm::createStructurizeCFGPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::Pass::from_inner(ret, true))
    }
}

pub fn create_tail_call_elimination_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createTailCallEliminationPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}

pub fn create_thread_sanitizer_pass() -> Option<::llvm::pass::FunctionPass> {
    unsafe {
        let ret = ::ffi::llvm::createThreadSanitizerPass();
        if ret.is_null() {
            return None;
        }
        Some(::llvm::pass::FunctionPass::from_inner(ret, true))
    }
}
