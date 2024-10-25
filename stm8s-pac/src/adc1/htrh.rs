#[doc = "Register `HTRH` reader"]
pub type R = crate::R<HTRH_SPEC>;
#[doc = "Register `HTRH` writer"]
pub type W = crate::W<HTRH_SPEC>;
#[doc = "Field `HT` reader - "]
pub type HT_R = crate::FieldReader;
#[doc = "Field `HT` writer - "]
pub type HT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<HTRH_SPEC> {
        HT_W::new(self, 0)
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
#[doc = "ADC high threshold register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HTRH_SPEC;
impl crate::RegisterSpec for HTRH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`htrh::R`](R) reader structure"]
impl crate::Readable for HTRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`htrh::W`](W) writer structure"]
impl crate::Writable for HTRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HTRH to value 0x03"]
impl crate::Resettable for HTRH_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
