use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "3" ></ circle > < line x1 = "3" y1 = "12" x2 = "9" y2 = "12" ></ line > < line x1 = "15" x2 = "21" y1 = "12" y2 = "12" ></ line > < / > } } pub const LUCIDE_GIT_COMMIT_HORIZONTAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24")] } ;