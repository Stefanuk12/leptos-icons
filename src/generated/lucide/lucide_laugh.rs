use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < path d = "M18 13a6 6 0 0 1-6 5 6 6 0 0 1-6-5h12Z" ></ path > < line y1 = "9" x2 = "9.01" y2 = "9" x1 = "9" ></ line > < line x1 = "15" y2 = "9" y1 = "9" x2 = "15.01" ></ line > < / > } } pub const LUCIDE_LAUGH : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round")] } ;