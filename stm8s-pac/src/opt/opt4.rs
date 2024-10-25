#[doc = "Register `OPT4` reader"]
pub type R = crate::R<OPT4_SPEC>;
#[doc = "Register `OPT4` writer"]
pub type W = crate::W<OPT4_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<OPT4_SPEC> {
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
#[doc = "Clock option\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opt4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opt4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPT4_SPEC;
impl crate::RegisterSpec for OPT4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`opt4::R`](R) reader structure"]
impl crate::Readable for OPT4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`opt4::W`](W) writer structure"]
impl crate::Writable for OPT4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPT4 to value 0"]
impl crate::Resettable for OPT4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
