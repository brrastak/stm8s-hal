#[doc = "Register `LTRH` reader"]
pub type R = crate::R<LTRH_SPEC>;
#[doc = "Register `LTRH` writer"]
pub type W = crate::W<LTRH_SPEC>;
#[doc = "Field `LT` reader - "]
pub type LT_R = crate::FieldReader;
#[doc = "Field `LT` writer - "]
pub type LT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn lt(&mut self) -> LT_W<LTRH_SPEC> {
        LT_W::new(self, 0)
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
#[doc = "ADC low threshold register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTRH_SPEC;
impl crate::RegisterSpec for LTRH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ltrh::R`](R) reader structure"]
impl crate::Readable for LTRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ltrh::W`](W) writer structure"]
impl crate::Writable for LTRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTRH to value 0"]
impl crate::Resettable for LTRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
