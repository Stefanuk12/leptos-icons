use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "8" cx = "11" cy = "11" ></ circle > < line y1 = "21" x1 = "21" x2 = "16.65" y2 = "16.65" ></ line > < line y1 = "11" y2 = "11" x2 = "14" x1 = "8" ></ line > < / > } } pub const LUCIDE_ZOOM_OUT : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none")] } ;