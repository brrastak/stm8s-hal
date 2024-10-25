#[doc = "Register `CCR1H` reader"]
pub type R = crate::R<CCR1H_SPEC>;
#[doc = "Register `CCR1H` writer"]
pub type W = crate::W<CCR1H_SPEC>;
#[doc = "Field `CCR1` reader - "]
pub type CCR1_R = crate::FieldReader;
#[doc = "Field `CCR1` writer - "]
pub type CCR1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ccr1(&self) -> CCR1_R {
        CCR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn ccr1(&mut self) -> CCR1_W<CCR1H_SPEC> {
        CCR1_W::new(self, 0)
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
#[doc = "TIM1 capture/compare register 1 high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR1H_SPEC;
impl crate::RegisterSpec for CCR1H_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ccr1h::R`](R) reader structure"]
impl crate::Readable for CCR1H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr1h::W`](W) writer structure"]
impl crate::Writable for CCR1H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR1H to value 0"]
impl crate::Resettable for CCR1H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
