use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "3" ></ circle > < line x1 = "3" x2 = "9" y1 = "12" y2 = "12" ></ line > < line y1 = "12" x1 = "15" y2 = "12" x2 = "21" ></ line > < / > } } pub const LUCIDE_GIT_COMMIT_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24")] } ;