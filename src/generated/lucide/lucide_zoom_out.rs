use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "11" cy = "11" r = "8" ></ circle > < line x1 = "21" y2 = "16.65" x2 = "16.65" y1 = "21" ></ line > < line x2 = "14" y2 = "11" x1 = "8" y1 = "11" ></ line > < / > } } pub const LUCIDE_ZOOM_OUT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;