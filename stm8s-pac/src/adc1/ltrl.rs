#[doc = "Register `LTRL` reader"]
pub type R = crate::R<LTRL_SPEC>;
#[doc = "Register `LTRL` writer"]
pub type W = crate::W<LTRL_SPEC>;
#[doc = "Field `LT` reader - "]
pub type LT_R = crate::FieldReader;
#[doc = "Field `LT` writer - "]
pub type LT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn lt(&mut self) -> LT_W<LTRL_SPEC> {
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
#[doc = "ADC low threshold register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTRL_SPEC;
impl crate::RegisterSpec for LTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ltrl::R`](R) reader structure"]
impl crate::Readable for LTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ltrl::W`](W) writer structure"]
impl crate::Writable for LTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTRL to value 0"]
impl crate::Resettable for LTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
