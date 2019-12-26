use std::mem;
use libipt_sys::pt_conf_addr_filter;


#[derive(Clone, Copy)]
pub enum AddrConfig {
    DISABLED = 0,
    FILTER   = 1,
    STOP     = 2
}

impl From<u32> for AddrConfig {
    fn from(cfg: u32) -> Self {
        match cfg {
            0 => AddrConfig::DISABLED,
            1 => AddrConfig::FILTER,
            2 => AddrConfig::STOP,
            _ => unreachable!()
        }
    }
}

/// an address range inside the address filter
#[derive(Clone, Copy)]
pub struct AddrRange {
    /// This corresponds to the IA32_RTIT_ADDRn_A MSRs
    a: u64,
    /// This corresponds to the IA32_RTIT_ADDRn_B MSRs
    b: u64,
    /// this corresponds to the respective fields in IA32_RTIT_CTL MSR
    cfg: AddrConfig
}

impl AddrRange {
    #[inline]
    pub fn new(a: u64, b: u64, cfg: AddrConfig) -> Self {
        AddrRange { a, b, cfg }
    }

    /// This corresponds to the IA32_RTIT_ADDRn_A MSRs
    #[inline]
    pub fn a(&self) -> u64 { self.a }
    /// This corresponds to the IA32_RTIT_ADDRn_B MSRs
    #[inline]
    pub fn b(&self) -> u64 { self.b }
    /// this corresponds to the respective fields in IA32_RTIT_CTL MSR
    #[inline]
    pub fn cfg(&self) -> AddrConfig { self.cfg }
    
    /// This corresponds to the IA32_RTIT_ADDRn_A MSRs
    #[inline]
    pub fn set_a(&mut self, a: u64) { self.a = a; }
    /// This corresponds to the IA32_RTIT_ADDRn_B MSRs
    #[inline]
    pub fn set_b(&mut self, b: u64) { self.b = b; }
    /// this corresponds to the respective fields in IA32_RTIT_CTL MSR
    #[inline]
    pub fn set_cfg(&mut self, cfg: AddrConfig) { self.cfg = cfg }
}

// could've written a macro, i know
// but its just like 4 variables i think its fine

/// the address filter configuration
#[derive(Clone, Copy)]
pub struct AddrFilter (pub(super) pt_conf_addr_filter);
impl AddrFilter {
    pub fn empty() -> Self { unsafe { mem::zeroed() }}
    pub fn new(addr0: Option<AddrRange>,
               addr1: Option<AddrRange>,
               addr2: Option<AddrRange>,
               addr3: Option<AddrRange>) -> Self {

        let mut filter = AddrFilter::empty();
        if let Some(a) = addr0 { filter.set_addr0(a); }
        if let Some(a) = addr1 { filter.set_addr1(a); }
        if let Some(a) = addr2 { filter.set_addr2(a); }
        if let Some(a) = addr3 { filter.set_addr3(a); }

        filter
    }

    #[inline]
    pub fn set_addr0(&mut self, range: AddrRange) {
        self.0.addr0_a = range.a;
        self.0.addr0_b = range.b;
        unsafe { self.0.config.ctl.set_addr0_cfg(range.cfg as u32) };
    }

    #[inline]
    pub fn set_addr1(&mut self, range: AddrRange) {
        self.0.addr1_a = range.a;
        self.0.addr1_b = range.b;
        unsafe { self.0.config.ctl.set_addr1_cfg(range.cfg as u32) };
    }

    #[inline]
    pub fn set_addr2(&mut self, range: AddrRange) {
        self.0.addr2_a = range.a;
        self.0.addr2_b = range.b;
        unsafe { self.0.config.ctl.set_addr2_cfg(range.cfg as u32) };
    }

    #[inline]
    pub fn set_addr3(&mut self, range: AddrRange) {
        self.0.addr3_a = range.a;
        self.0.addr3_b = range.b;
        unsafe { self.0.config.ctl.set_addr3_cfg(range.cfg as u32) };
    }

    #[inline]
    pub fn addr0(&self) -> AddrRange {
        unsafe {
            AddrRange::new(self.0.addr0_a, self.0.addr0_b,
                self.0.config.ctl.addr0_cfg().into())
        }
    }

    #[inline]
    pub fn addr1(&self) -> AddrRange {
        unsafe {
            AddrRange::new(self.0.addr1_a, self.0.addr1_b,
                self.0.config.ctl.addr1_cfg().into())
        }
    }

    #[inline]
    pub fn addr2(&self) -> AddrRange {
        unsafe {
            AddrRange::new(self.0.addr2_a, self.0.addr2_b,
                self.0.config.ctl.addr2_cfg().into())
        }
    }

    #[inline]
    pub fn addr3(&self) -> AddrRange {
        unsafe {
            AddrRange::new(self.0.addr3_a, self.0.addr3_b,
                self.0.config.ctl.addr3_cfg().into())
        }
    }
}