use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cy = "12" cx = "12" ></ circle > < line y1 = "12" x2 = "9" y2 = "12" x1 = "3" ></ line > < line y2 = "12" x1 = "15" x2 = "21" y1 = "12" ></ line > < / > } } pub const LUCIDE_GIT_COMMIT_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;