use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "10" ></ circle > < path d = "M18 13a6 6 0 0 1-6 5 6 6 0 0 1-6-5h12Z" ></ path > < line y1 = "9" x1 = "9" x2 = "9.01" y2 = "9" ></ line > < line x1 = "15" y2 = "9" y1 = "9" x2 = "15.01" ></ line > < / > } } pub const LUCIDE_LAUGH : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;