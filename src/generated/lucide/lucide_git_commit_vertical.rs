use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 3v6" ></ path > < circle cx = "12" cy = "12" r = "3" ></ circle > < path d = "M12 15v6" ></ path > < / > } } pub const LUCIDE_GIT_COMMIT_VERTICAL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;