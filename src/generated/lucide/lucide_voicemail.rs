use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "6" r = "4" cy = "12" ></ circle > < circle cy = "12" r = "4" cx = "18" ></ circle > < line x2 = "18" y1 = "16" x1 = "6" y2 = "16" ></ line > < / > } } pub const LUCIDE_VOICEMAIL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;