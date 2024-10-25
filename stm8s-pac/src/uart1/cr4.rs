#[doc = "Register `CR4` reader"]
pub type R = crate::R<CR4_SPEC>;
#[doc = "Register `CR4` writer"]
pub type W = crate::W<CR4_SPEC>;
#[doc = "Field `ADD` reader - "]
pub type ADD_R = crate::FieldReader;
#[doc = "Field `ADD` writer - "]
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LBDF` reader - "]
pub type LBDF_R = crate::BitReader;
#[doc = "Field `LBDF` writer - "]
pub type LBDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDL` reader - "]
pub type LBDL_R = crate::BitReader;
#[doc = "Field `LBDL` writer - "]
pub type LBDL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDIEN` reader - "]
pub type LBDIEN_R = crate::BitReader;
#[doc = "Field `LBDIEN` writer - "]
pub type LBDIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lbdf(&self) -> LBDF_R {
        LBDF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lbdl(&self) -> LBDL_R {
        LBDL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lbdien(&self) -> LBDIEN_R {
        LBDIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<CR4_SPEC> {
        ADD_W::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn lbdf(&mut self) -> LBDF_W<CR4_SPEC> {
        LBDF_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn lbdl(&mut self) -> LBDL_W<CR4_SPEC> {
        LBDL_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn lbdien(&mut self) -> LBDIEN_W<CR4_SPEC> {
        LBDIEN_W::new(self, 6)
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
#[doc = "UART1 control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR4_SPEC;
impl crate::RegisterSpec for CR4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cr4::R`](R) reader structure"]
impl crate::Readable for CR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr4::W`](W) writer structure"]
impl crate::Writable for CR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR4 to value 0"]
impl crate::Resettable for CR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
