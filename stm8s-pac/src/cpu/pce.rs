#[doc = "Register `PCE` reader"]
pub type R = crate::R<PCE_SPEC>;
#[doc = "Register `PCE` writer"]
pub type W = crate::W<PCE_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PCE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Program counter extended\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pce::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pce::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCE_SPEC;
impl crate::RegisterSpec for PCE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pce::R`](R) reader structure"]
impl crate::Readable for PCE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pce::W`](W) writer structure"]
impl crate::Writable for PCE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCE to value 0"]
impl crate::Resettable for PCE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
