use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "5" x2 = "5" y2 = "19" x1 = "19" ></ line > < circle r = "2.5" cx = "6.5" cy = "6.5" ></ circle > < circle cx = "17.5" r = "2.5" cy = "17.5" ></ circle > < / > } } pub const LUCIDE_PERCENT : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;