use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "8" cx = "11" cy = "11" ></ circle > < line y2 = "16.65" x1 = "21" y1 = "21" x2 = "16.65" ></ line > < line x2 = "11" y1 = "8" y2 = "14" x1 = "11" ></ line > < line x1 = "8" y1 = "11" x2 = "14" y2 = "11" ></ line > < / > } } pub const LUCIDE_ZOOM_IN : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;