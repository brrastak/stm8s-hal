#[doc = "Register `IDR` reader"]
pub type R = crate::R<IDR_SPEC>;
#[doc = "Register `IDR` writer"]
pub type W = crate::W<IDR_SPEC>;
#[doc = "Field `IDR0` reader - "]
pub type IDR0_R = crate::BitReader;
#[doc = "Field `IDR0` writer - "]
pub type IDR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDR1` reader - "]
pub type IDR1_R = crate::BitReader;
#[doc = "Field `IDR1` writer - "]
pub type IDR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDR2` reader - "]
pub type IDR2_R = crate::BitReader;
#[doc = "Field `IDR2` writer - "]
pub type IDR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDR3` reader - "]
pub type IDR3_R = crate::BitReader;
#[doc = "Field `IDR3` writer - "]
pub type IDR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDR4` reader - "]
pub type IDR4_R = crate::BitReader;
#[doc = "Field `IDR4` writer - "]
pub type IDR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDR5` reader - "]
pub type IDR5_R = crate::BitReader;
#[doc = "Field `IDR5` writer - "]
pub type IDR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDR6` reader - "]
pub type IDR6_R = crate::BitReader;
#[doc = "Field `IDR6` writer - "]
pub type IDR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDR7` reader - "]
pub type IDR7_R = crate::BitReader;
#[doc = "Field `IDR7` writer - "]
pub type IDR7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn idr0(&self) -> IDR0_R {
        IDR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn idr1(&self) -> IDR1_R {
        IDR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn idr2(&self) -> IDR2_R {
        IDR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn idr3(&self) -> IDR3_R {
        IDR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn idr4(&self) -> IDR4_R {
        IDR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn idr5(&self) -> IDR5_R {
        IDR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn idr6(&self) -> IDR6_R {
        IDR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn idr7(&self) -> IDR7_R {
        IDR7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn idr0(&mut self) -> IDR0_W<IDR_SPEC> {
        IDR0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn idr1(&mut self) -> IDR1_W<IDR_SPEC> {
        IDR1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn idr2(&mut self) -> IDR2_W<IDR_SPEC> {
        IDR2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn idr3(&mut self) -> IDR3_W<IDR_SPEC> {
        IDR3_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn idr4(&mut self) -> IDR4_W<IDR_SPEC> {
        IDR4_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn idr5(&mut self) -> IDR5_W<IDR_SPEC> {
        IDR5_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn idr6(&mut self) -> IDR6_W<IDR_SPEC> {
        IDR6_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn idr7(&mut self) -> IDR7_W<IDR_SPEC> {
        IDR7_W::new(self, 7)
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
#[doc = "Port C input pin value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`idr::R`](R) reader structure"]
impl crate::Readable for IDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
