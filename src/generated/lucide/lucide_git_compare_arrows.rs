use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cy = "6" cx = "5" ></ circle > < path d = "M12 6h5a2 2 0 0 1 2 2v7" ></ path > < path d = "m15 9-3-3 3-3" ></ path > < circle cy = "18" r = "3" cx = "19" ></ circle > < path d = "M12 18H7a2 2 0 0 1-2-2V9" ></ path > < path d = "m9 15 3 3-3 3" ></ path > < / > } } pub const LUCIDE_GIT_COMPARE_ARROWS : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;