#[doc = "Register `OARL` reader"]
pub type R = crate::R<OARL_SPEC>;
#[doc = "Register `OARL` writer"]
pub type W = crate::W<OARL_SPEC>;
#[doc = "Field `ADD0` reader - "]
pub type ADD0_R = crate::BitReader;
#[doc = "Field `ADD0` writer - "]
pub type ADD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD` reader - "]
pub type ADD_R = crate::FieldReader;
#[doc = "Field `ADD` writer - "]
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn add0(&self) -> ADD0_R {
        ADD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn add0(&mut self) -> ADD0_W<OARL_SPEC> {
        ADD0_W::new(self, 0)
    }
    #[doc = "Bits 1:7"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<OARL_SPEC> {
        ADD_W::new(self, 1)
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
#[doc = "I2C Own address register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oarl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oarl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OARL_SPEC;
impl crate::RegisterSpec for OARL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`oarl::R`](R) reader structure"]
impl crate::Readable for OARL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oarl::W`](W) writer structure"]
impl crate::Writable for OARL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OARL to value 0"]
impl crate::Resettable for OARL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
