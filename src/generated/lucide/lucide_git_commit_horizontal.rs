use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "3" cx = "12" ></ circle > < line x1 = "3" y2 = "12" x2 = "9" y1 = "12" ></ line > < line y2 = "12" x1 = "15" y1 = "12" x2 = "21" ></ line > < / > } } pub const LUCIDE_GIT_COMMIT_HORIZONTAL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24")] } ;