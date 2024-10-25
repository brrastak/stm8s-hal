#[doc = "Register `CCR3H` reader"]
pub type R = crate::R<CCR3H_SPEC>;
#[doc = "Register `CCR3H` writer"]
pub type W = crate::W<CCR3H_SPEC>;
#[doc = "Field `CCR3` reader - "]
pub type CCR3_R = crate::FieldReader;
#[doc = "Field `CCR3` writer - "]
pub type CCR3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ccr3(&self) -> CCR3_R {
        CCR3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn ccr3(&mut self) -> CCR3_W<CCR3H_SPEC> {
        CCR3_W::new(self, 0)
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
#[doc = "TIM2 capture/compare register 3 high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr3h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr3h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR3H_SPEC;
impl crate::RegisterSpec for CCR3H_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ccr3h::R`](R) reader structure"]
impl crate::Readable for CCR3H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr3h::W`](W) writer structure"]
impl crate::Writable for CCR3H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR3H to value 0"]
impl crate::Resettable for CCR3H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
