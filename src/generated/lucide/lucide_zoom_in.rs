use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "11" cy = "11" r = "8" ></ circle > < line y2 = "16.65" x1 = "21" y1 = "21" x2 = "16.65" ></ line > < line x2 = "11" x1 = "11" y1 = "8" y2 = "14" ></ line > < line y2 = "11" x2 = "14" y1 = "11" x1 = "8" ></ line > < / > } } pub const LUCIDE_ZOOM_IN : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor")] } ;