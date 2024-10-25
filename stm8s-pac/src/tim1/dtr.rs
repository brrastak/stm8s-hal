#[doc = "Register `DTR` reader"]
pub type R = crate::R<DTR_SPEC>;
#[doc = "Register `DTR` writer"]
pub type W = crate::W<DTR_SPEC>;
#[doc = "Field `DTG` reader - "]
pub type DTG_R = crate::FieldReader;
#[doc = "Field `DTG` writer - "]
pub type DTG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn dtg(&mut self) -> DTG_W<DTR_SPEC> {
        DTG_W::new(self, 0)
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
#[doc = "TIM1 dead-time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTR_SPEC;
impl crate::RegisterSpec for DTR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dtr::R`](R) reader structure"]
impl crate::Readable for DTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dtr::W`](W) writer structure"]
impl crate::Writable for DTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTR to value 0"]
impl crate::Resettable for DTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
