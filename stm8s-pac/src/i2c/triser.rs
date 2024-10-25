#[doc = "Register `TRISER` reader"]
pub type R = crate::R<TRISER_SPEC>;
#[doc = "Register `TRISER` writer"]
pub type W = crate::W<TRISER_SPEC>;
#[doc = "Field `TRISE` reader - "]
pub type TRISE_R = crate::FieldReader;
#[doc = "Field `TRISE` writer - "]
pub type TRISE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn trise(&self) -> TRISE_R {
        TRISE_R::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn trise(&mut self) -> TRISE_W<TRISER_SPEC> {
        TRISE_W::new(self, 0)
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
#[doc = "I2C TRISE register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`triser::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`triser::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRISER_SPEC;
impl crate::RegisterSpec for TRISER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`triser::R`](R) reader structure"]
impl crate::Readable for TRISER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`triser::W`](W) writer structure"]
impl crate::Writable for TRISER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRISER to value 0x02"]
impl crate::Resettable for TRISER_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
