use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "6" r = "4" ></ circle > < circle cx = "18" cy = "12" r = "4" ></ circle > < line x2 = "18" y1 = "16" x1 = "6" y2 = "16" ></ line > < / > } } pub const LUCIDE_VOICEMAIL : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;