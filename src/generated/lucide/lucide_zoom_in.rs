use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "11" r = "8" cy = "11" ></ circle > < line x2 = "16.65" x1 = "21" y2 = "16.65" y1 = "21" ></ line > < line y1 = "8" x1 = "11" x2 = "11" y2 = "14" ></ line > < line x1 = "8" y2 = "11" y1 = "11" x2 = "14" ></ line > < / > } } pub const LUCIDE_ZOOM_IN : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;