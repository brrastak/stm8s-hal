#[doc = "Register `SWCR` reader"]
pub type R = crate::R<SWCR_SPEC>;
#[doc = "Register `SWCR` writer"]
pub type W = crate::W<SWCR_SPEC>;
#[doc = "Field `SWBSY` reader - "]
pub type SWBSY_R = crate::BitReader;
#[doc = "Field `SWBSY` writer - "]
pub type SWBSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWEN` reader - "]
pub type SWEN_R = crate::BitReader;
#[doc = "Field `SWEN` writer - "]
pub type SWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIEN` reader - "]
pub type SWIEN_R = crate::BitReader;
#[doc = "Field `SWIEN` writer - "]
pub type SWIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIF` reader - "]
pub type SWIF_R = crate::BitReader;
#[doc = "Field `SWIF` writer - "]
pub type SWIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swbsy(&self) -> SWBSY_R {
        SWBSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swen(&self) -> SWEN_R {
        SWEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn swien(&self) -> SWIEN_R {
        SWIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn swif(&self) -> SWIF_R {
        SWIF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn swbsy(&mut self) -> SWBSY_W<SWCR_SPEC> {
        SWBSY_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn swen(&mut self) -> SWEN_W<SWCR_SPEC> {
        SWEN_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn swien(&mut self) -> SWIEN_W<SWCR_SPEC> {
        SWIEN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn swif(&mut self) -> SWIF_W<SWCR_SPEC> {
        SWIF_W::new(self, 3)
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
#[doc = "Clock switch control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWCR_SPEC;
impl crate::RegisterSpec for SWCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`swcr::R`](R) reader structure"]
impl crate::Readable for SWCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swcr::W`](W) writer structure"]
impl crate::Writable for SWCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWCR to value 0"]
impl crate::Resettable for SWCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
