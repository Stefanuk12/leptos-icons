use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "3" ></ circle > < line y2 = "12" x1 = "3" x2 = "9" y1 = "12" ></ line > < line y1 = "12" y2 = "12" x2 = "21" x1 = "15" ></ line > < / > } } pub const LUCIDE_GIT_COMMIT_HORIZONTAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2")] } ;