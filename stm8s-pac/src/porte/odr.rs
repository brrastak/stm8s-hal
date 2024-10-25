#[doc = "Register `ODR` reader"]
pub type R = crate::R<ODR_SPEC>;
#[doc = "Register `ODR` writer"]
pub type W = crate::W<ODR_SPEC>;
#[doc = "Field `ODR0` reader - "]
pub type ODR0_R = crate::BitReader;
#[doc = "Field `ODR0` writer - "]
pub type ODR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODR1` reader - "]
pub type ODR1_R = crate::BitReader;
#[doc = "Field `ODR1` writer - "]
pub type ODR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODR2` reader - "]
pub type ODR2_R = crate::BitReader;
#[doc = "Field `ODR2` writer - "]
pub type ODR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODR3` reader - "]
pub type ODR3_R = crate::BitReader;
#[doc = "Field `ODR3` writer - "]
pub type ODR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODR4` reader - "]
pub type ODR4_R = crate::BitReader;
#[doc = "Field `ODR4` writer - "]
pub type ODR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODR5` reader - "]
pub type ODR5_R = crate::BitReader;
#[doc = "Field `ODR5` writer - "]
pub type ODR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODR6` reader - "]
pub type ODR6_R = crate::BitReader;
#[doc = "Field `ODR6` writer - "]
pub type ODR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODR7` reader - "]
pub type ODR7_R = crate::BitReader;
#[doc = "Field `ODR7` writer - "]
pub type ODR7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn odr0(&self) -> ODR0_R {
        ODR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn odr1(&self) -> ODR1_R {
        ODR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn odr2(&self) -> ODR2_R {
        ODR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn odr3(&self) -> ODR3_R {
        ODR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn odr4(&self) -> ODR4_R {
        ODR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn odr5(&self) -> ODR5_R {
        ODR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn odr6(&self) -> ODR6_R {
        ODR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn odr7(&self) -> ODR7_R {
        ODR7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn odr0(&mut self) -> ODR0_W<ODR_SPEC> {
        ODR0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn odr1(&mut self) -> ODR1_W<ODR_SPEC> {
        ODR1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn odr2(&mut self) -> ODR2_W<ODR_SPEC> {
        ODR2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn odr3(&mut self) -> ODR3_W<ODR_SPEC> {
        ODR3_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn odr4(&mut self) -> ODR4_W<ODR_SPEC> {
        ODR4_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn odr5(&mut self) -> ODR5_W<ODR_SPEC> {
        ODR5_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn odr6(&mut self) -> ODR6_W<ODR_SPEC> {
        ODR6_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn odr7(&mut self) -> ODR7_W<ODR_SPEC> {
        ODR7_W::new(self, 7)
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
#[doc = "Port E data output latch register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODR_SPEC;
impl crate::RegisterSpec for ODR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`odr::R`](R) reader structure"]
impl crate::Readable for ODR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`odr::W`](W) writer structure"]
impl crate::Writable for ODR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ODR to value 0"]
impl crate::Resettable for ODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
