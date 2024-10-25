#[doc = "Register `ICKR` reader"]
pub type R = crate::R<ICKR_SPEC>;
#[doc = "Register `ICKR` writer"]
pub type W = crate::W<ICKR_SPEC>;
#[doc = "Field `HSIEN` reader - "]
pub type HSIEN_R = crate::BitReader;
#[doc = "Field `HSIEN` writer - "]
pub type HSIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDY` reader - "]
pub type HSIRDY_R = crate::BitReader;
#[doc = "Field `HSIRDY` writer - "]
pub type HSIRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FHW` reader - "]
pub type FHW_R = crate::BitReader;
#[doc = "Field `FHW` writer - "]
pub type FHW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIEN` reader - "]
pub type LSIEN_R = crate::BitReader;
#[doc = "Field `LSIEN` writer - "]
pub type LSIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDY` reader - "]
pub type LSIRDY_R = crate::BitReader;
#[doc = "Field `LSIRDY` writer - "]
pub type LSIRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGAH` reader - "]
pub type REGAH_R = crate::BitReader;
#[doc = "Field `REGAH` writer - "]
pub type REGAH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hsien(&self) -> HSIEN_R {
        HSIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fhw(&self) -> FHW_R {
        FHW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lsien(&self) -> LSIEN_R {
        LSIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn regah(&self) -> REGAH_R {
        REGAH_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn hsien(&mut self) -> HSIEN_W<ICKR_SPEC> {
        HSIEN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdy(&mut self) -> HSIRDY_W<ICKR_SPEC> {
        HSIRDY_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn fhw(&mut self) -> FHW_W<ICKR_SPEC> {
        FHW_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn lsien(&mut self) -> LSIEN_W<ICKR_SPEC> {
        LSIEN_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdy(&mut self) -> LSIRDY_W<ICKR_SPEC> {
        LSIRDY_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn regah(&mut self) -> REGAH_W<ICKR_SPEC> {
        REGAH_W::new(self, 5)
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
#[doc = "Internal clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ickr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ickr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICKR_SPEC;
impl crate::RegisterSpec for ICKR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ickr::R`](R) reader structure"]
impl crate::Readable for ICKR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ickr::W`](W) writer structure"]
impl crate::Writable for ICKR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICKR to value 0x01"]
impl crate::Resettable for ICKR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
