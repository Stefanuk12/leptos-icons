use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 6h5a2 2 0 0 1 2 2v7" ></ path > < path d = "m15 9-3-3 3-3" ></ path > < circle cy = "18" cx = "19" r = "3" ></ circle > < path d = "M12 18H7a2 2 0 0 1-2-2V9" ></ path > < path d = "m9 15 3 3-3 3" ></ path > < / > } } pub const LucideGitCompareArrows : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;