use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 3v6" ></ path > < circle r = "3" cx = "12" cy = "12" ></ circle > < path d = "M12 15v6" ></ path > < / > } } pub const LUCIDE_GIT_COMMIT_VERTICAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor")] } ;