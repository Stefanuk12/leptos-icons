use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "6" r = "4" ></ circle > < circle r = "4" cx = "18" cy = "12" ></ circle > < line x1 = "6" y2 = "16" x2 = "18" y1 = "16" ></ line > < / > } } pub const LUCIDE_VOICEMAIL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;