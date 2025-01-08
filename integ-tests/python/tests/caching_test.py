import pytest
import uuid
import time
from .test_setup import b

@pytest.mark.asyncio
async def test_caching():
    story_idea = f"""
In a futuristic world where dreams are a marketable asset and collective experience, an introverted and socially inept teenager named Alex realizes they have a unique and potent skill to not only observe but also alter the dreams of others. Initially excited by this newfound talent, Alex starts discreetly modifying the dreams of peers and relatives, aiding them in conquering fears, boosting self-esteem, or embarking on fantastical journeys. As Alex's abilities expand, so does their sway. They begin marketing exclusive dream experiences on the underground market, designing complex and captivating dreamscapes for affluent clients. However, the boundary between dream and reality starts to fade for those subjected to Alex's creations. Some clients find it difficult to distinguish between their genuine memories and the fabricated ones inserted by Alex's dream manipulation.

Challenges emerge when a secretive government organization becomes aware of Alex's distinct talents. They propose Alex utilize their gift for "the greater good," suggesting uses in therapy, criminal reform, and even national defense. Concurrently, a covert resistance group contacts Alex, cautioning them about the risks of dream manipulation and the potential for widespread control and exploitation. Trapped between these conflicting forces, Alex must navigate a tangled web of moral dilemmas. They wrestle with issues of free will, the essence of consciousness, and the duty that comes with having influence over people's minds. As the repercussions of their actions ripple outward, impacting the lives of loved ones and strangers alike, Alex is compelled to face the true nature of their power and decide how—or if—it should be wielded.

The narrative investigates themes of identity, the subconscious, the ethics of technology, and the power of creativity. It explores the possible outcomes of a world where our most intimate thoughts and experiences are no longer truly our own, and scrutinizes the fine line between aiding others and manipulating them for personal benefit or a perceived greater good. The story further delves into the societal ramifications of such abilities, questioning the moral limits of altering consciousness and the potential for misuse in a world where dreams can be commercialized. It challenges the reader to contemplate the impact of technology on personal freedom and the ethical duties of those who wield such power.

As Alex's journey progresses, they meet various individuals whose lives have been influenced by their dream manipulations, each offering a distinct viewpoint on the ethical issues at hand. From a peer who gains newfound confidence to a wealthy client who becomes dependent on the dreamscapes, the ripple effects of Alex's actions are significant and extensive. The government agency's interest in Alex's abilities raises questions about the potential for state control and surveillance, while the resistance movement underscores the dangers of unchecked power and the necessity of protecting individual freedoms.

Ultimately, Alex's story is one of self-discovery and moral reflection, as they must choose whether to use their abilities for personal gain, align with the government's vision of a controlled utopia, or join the resistance in their struggle for freedom and autonomy. The narrative encourages readers to reflect on the nature of reality, the boundaries of human experience, and the ethical implications of a world where dreams are no longer private sanctuaries but shared and manipulated commodities. It also examines the psychological impact on Alex, who must cope with the burden of knowing the intimate fears and desires of others, and the isolation that comes from being unable to share their own dreams without altering them.

The story further investigates the technological progress that has made dream manipulation feasible, questioning the role of innovation in society and the potential for both advancement and peril. It considers the societal divide between those who can afford to purchase enhanced dream experiences and those who cannot, highlighting issues of inequality and access. As Alex becomes more ensnared in the web of their own making, they must confront the possibility that their actions could lead to unintended consequences, not just for themselves but for the fabric of society as a whole.

In the end, Alex's journey is a cautionary tale about the power of dreams and the responsibilities that come with wielding such influence. It serves as a reminder of the importance of ethical considerations in the face of technological advancement and the need to balance innovation with humanity. The story leaves readers pondering the true cost of a world where dreams are no longer sacred, and the potential for both wonder and danger in the uncharted territories of the mind. But it's also a story about the power of imagination and the potential for change, even in a world where our deepest thoughts are no longer our own. And it's a story about the power of choice, and the importance of fighting for the freedom to dream.

In conclusion, this story is a reflection on the power of dreams and the responsibilities that come with wielding such influence. It serves as a reminder of the importance of ethical considerations in the face of technological advancement and the need to balance innovation with humanity. The story leaves readers pondering the true cost of a world where dreams are no longer sacred, and the potential for both wonder and danger in the uncharted territories of the mind. But it's also a story about the power of imagination and the potential for change, even in a world where our deepest thoughts are no longer our own. And it's a story about the power of choice, and the importance of fighting for the freedom to dream.
"""
    rand = uuid.uuid4().hex
    story_idea = rand + story_idea

    start = time.time()
    _ = await b.TestCaching(story_idea, "1. try to be funny")
    duration = time.time() - start

    start = time.time()
    _ = await b.TestCaching(story_idea, "1. try to be funny")
    duration2 = time.time() - start

    print("Duration no caching: ", duration)
    print("Duration with caching: ", duration2)

    # Allow for some timing variation but ensure caching is significantly faster
    assert duration2 < duration * 0.95, f"Expected second call ({duration2}s) to be at least 5% faster than first call ({duration}s)"