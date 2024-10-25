#[doc = "Register `SR3` reader"]
pub type R = crate::R<SR3_SPEC>;
#[doc = "Register `SR3` writer"]
pub type W = crate::W<SR3_SPEC>;
#[doc = "Field `MSL` reader - "]
pub type MSL_R = crate::BitReader;
#[doc = "Field `MSL` writer - "]
pub type MSL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - "]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `BUSY` writer - "]
pub type BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRA` reader - "]
pub type TRA_R = crate::BitReader;
#[doc = "Field `TRA` writer - "]
pub type TRA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENCALL` reader - "]
pub type GENCALL_R = crate::BitReader;
#[doc = "Field `GENCALL` writer - "]
pub type GENCALL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn msl(&self) -> MSL_R {
        MSL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tra(&self) -> TRA_R {
        TRA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gencall(&self) -> GENCALL_R {
        GENCALL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn msl(&mut self) -> MSL_W<SR3_SPEC> {
        MSL_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<SR3_SPEC> {
        BUSY_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tra(&mut self) -> TRA_W<SR3_SPEC> {
        TRA_W::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn gencall(&mut self) -> GENCALL_W<SR3_SPEC> {
        GENCALL_W::new(self, 4)
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
#[doc = "I2C status register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR3_SPEC;
impl crate::RegisterSpec for SR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sr3::R`](R) reader structure"]
impl crate::Readable for SR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr3::W`](W) writer structure"]
impl crate::Writable for SR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR3 to value 0"]
impl crate::Resettable for SR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
