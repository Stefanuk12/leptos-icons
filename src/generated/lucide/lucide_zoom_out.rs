use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "8" cy = "11" cx = "11" ></ circle > < line y2 = "16.65" x1 = "21" y1 = "21" x2 = "16.65" ></ line > < line x2 = "14" y2 = "11" y1 = "11" x1 = "8" ></ line > < / > } } pub const LUCIDE_ZOOM_OUT : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;