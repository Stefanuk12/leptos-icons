use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "6" r = "1" ></ circle > < line x2 = "19" y2 = "12" y1 = "12" x1 = "5" ></ line > < circle cx = "12" r = "1" cy = "18" ></ circle > < / > } } pub const LUCIDE_DIVIDE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;