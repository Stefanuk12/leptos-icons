use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "5" y1 = "5" y2 = "19" x1 = "19" ></ line > < circle cy = "6.5" cx = "6.5" r = "2.5" ></ circle > < circle r = "2.5" cx = "17.5" cy = "17.5" ></ circle > < / > } } pub const LUCIDE_PERCENT : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("width" , "24")] } ;