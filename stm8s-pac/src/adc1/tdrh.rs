#[doc = "Register `TDRH` reader"]
pub type R = crate::R<TDRH_SPEC>;
#[doc = "Register `TDRH` writer"]
pub type W = crate::W<TDRH_SPEC>;
#[doc = "Field `TD` reader - "]
pub type TD_R = crate::FieldReader;
#[doc = "Field `TD` writer - "]
pub type TD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn td(&self) -> TD_R {
        TD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn td(&mut self) -> TD_W<TDRH_SPEC> {
        TD_W::new(self, 0)
    }
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
#[doc = "ADC Schmitt trigger disable register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDRH_SPEC;
impl crate::RegisterSpec for TDRH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tdrh::R`](R) reader structure"]
impl crate::Readable for TDRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tdrh::W`](W) writer structure"]
impl crate::Writable for TDRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TDRH to value 0"]
impl crate::Resettable for TDRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
