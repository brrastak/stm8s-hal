#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Field `PAIS` reader - "]
pub type PAIS_R = crate::FieldReader;
#[doc = "Field `PAIS` writer - "]
pub type PAIS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PBIS` reader - "]
pub type PBIS_R = crate::FieldReader;
#[doc = "Field `PBIS` writer - "]
pub type PBIS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCIS` reader - "]
pub type PCIS_R = crate::FieldReader;
#[doc = "Field `PCIS` writer - "]
pub type PCIS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PDIS` reader - "]
pub type PDIS_R = crate::FieldReader;
#[doc = "Field `PDIS` writer - "]
pub type PDIS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pais(&self) -> PAIS_R {
        PAIS_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pbis(&self) -> PBIS_R {
        PBIS_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pcis(&self) -> PCIS_R {
        PCIS_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pdis(&self) -> PDIS_R {
        PDIS_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pais(&mut self) -> PAIS_W<CR1_SPEC> {
        PAIS_W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn pbis(&mut self) -> PBIS_W<CR1_SPEC> {
        PBIS_W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn pcis(&mut self) -> PCIS_W<CR1_SPEC> {
        PCIS_W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn pdis(&mut self) -> PDIS_W<CR1_SPEC> {
        PDIS_W::new(self, 6)
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
#[doc = "External interrupt control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
