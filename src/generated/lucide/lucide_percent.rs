use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "19" x2 = "5" y1 = "5" y2 = "19" ></ line > < circle cy = "6.5" r = "2.5" cx = "6.5" ></ circle > < circle cx = "17.5" r = "2.5" cy = "17.5" ></ circle > < / > } } pub const LUCIDE_PERCENT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24")] } ;