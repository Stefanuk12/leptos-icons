use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < path d = "M18 13a6 6 0 0 1-6 5 6 6 0 0 1-6-5h12Z" ></ path > < line x1 = "9" y2 = "9" x2 = "9.01" y1 = "9" ></ line > < line x2 = "15.01" x1 = "15" y1 = "9" y2 = "9" ></ line > < / > } } pub const LUCIDE_LAUGH : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;