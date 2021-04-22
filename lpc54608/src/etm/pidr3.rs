#[doc = "Register `PIDR3` reader"]
pub struct R(crate::R<PIDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PIDR3_SPEC>> for R {
    fn from(reader: crate::R<PIDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CustomerModified` reader - Customer Modified."]
pub struct CUSTOMERMODIFIED_R(crate::FieldReader<u8, u8>);
impl CUSTOMERMODIFIED_R {
    pub(crate) fn new(bits: u8) -> Self {
        CUSTOMERMODIFIED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CUSTOMERMODIFIED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RevAnd` reader - RevAnd"]
pub struct REVAND_R(crate::FieldReader<u8, u8>);
impl REVAND_R {
    pub(crate) fn new(bits: u8) -> Self {
        REVAND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVAND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Customer Modified."]
    #[inline(always)]
    pub fn customer_modified(&self) -> CUSTOMERMODIFIED_R {
        CUSTOMERMODIFIED_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RevAnd"]
    #[inline(always)]
    pub fn rev_and(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral Identification Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr3](index.html) module"]
pub struct PIDR3_SPEC;
impl crate::RegisterSpec for PIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr3::R](R) reader structure"]
impl crate::Readable for PIDR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR3 to value 0"]
impl crate::Resettable for PIDR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
