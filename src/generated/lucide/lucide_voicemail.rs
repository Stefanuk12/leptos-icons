use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "4" cx = "6" cy = "12" ></ circle > < circle cx = "18" cy = "12" r = "4" ></ circle > < line y1 = "16" y2 = "16" x1 = "6" x2 = "18" ></ line > < / > } } pub const LUCIDE_VOICEMAIL : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor")] } ;