use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "11" r = "8" cy = "11" ></ circle > < line x1 = "21" y1 = "21" x2 = "16.65" y2 = "16.65" ></ line > < line x2 = "11" y2 = "14" x1 = "11" y1 = "8" ></ line > < line x2 = "14" y2 = "11" x1 = "8" y1 = "11" ></ line > < / > } } pub const LUCIDE_ZOOM_IN : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;