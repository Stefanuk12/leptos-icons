use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "5" x1 = "19" y1 = "5" y2 = "19" ></ line > < circle r = "2.5" cy = "6.5" cx = "6.5" ></ circle > < circle r = "2.5" cx = "17.5" cy = "17.5" ></ circle > < / > } } pub const LUCIDE_PERCENT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;