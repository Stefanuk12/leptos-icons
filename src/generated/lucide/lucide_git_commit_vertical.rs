use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 3v6" ></ path > < circle cy = "12" r = "3" cx = "12" ></ circle > < path d = "M12 15v6" ></ path > < / > } } pub const LUCIDE_GIT_COMMIT_VERTICAL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;