use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "8" cy = "11" cx = "11" ></ circle > < line x1 = "21" y2 = "16.65" y1 = "21" x2 = "16.65" ></ line > < line y1 = "8" x2 = "11" x1 = "11" y2 = "14" ></ line > < line x1 = "8" y1 = "11" y2 = "11" x2 = "14" ></ line > < / > } } pub const LUCIDE_ZOOM_IN : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;