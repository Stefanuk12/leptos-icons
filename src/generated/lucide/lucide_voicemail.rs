use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "6" cy = "12" r = "4" ></ circle > < circle cy = "12" r = "4" cx = "18" ></ circle > < line y1 = "16" x1 = "6" y2 = "16" x2 = "18" ></ line > < / > } } pub const LUCIDE_VOICEMAIL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;