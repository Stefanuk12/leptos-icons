use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < path d = "M18 13a6 6 0 0 1-6 5 6 6 0 0 1-6-5h12Z" ></ path > < line x2 = "9.01" x1 = "9" y1 = "9" y2 = "9" ></ line > < line y2 = "9" x1 = "15" x2 = "15.01" y1 = "9" ></ line > < / > } } pub const LUCIDE_LAUGH : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2")] } ;