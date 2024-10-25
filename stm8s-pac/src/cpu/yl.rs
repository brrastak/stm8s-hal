#[doc = "Register `YL` reader"]
pub type R = crate::R<YL_SPEC>;
#[doc = "Register `YL` writer"]
pub type W = crate::W<YL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<YL_SPEC> {
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
#[doc = "Y index register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`yl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`yl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct YL_SPEC;
impl crate::RegisterSpec for YL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`yl::R`](R) reader structure"]
impl crate::Readable for YL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`yl::W`](W) writer structure"]
impl crate::Writable for YL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets YL to value 0"]
impl crate::Resettable for YL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
