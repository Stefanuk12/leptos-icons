use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "5" y2 = "19" x2 = "5" x1 = "19" ></ line > < circle cy = "6.5" r = "2.5" cx = "6.5" ></ circle > < circle cx = "17.5" cy = "17.5" r = "2.5" ></ circle > < / > } } pub const LUCIDE_PERCENT : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;